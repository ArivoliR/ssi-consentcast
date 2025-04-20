import streamlit as st
import json
import requests

# 🔧 Page settings
st.set_page_config(page_title="Verifier Portal",
                   page_icon="🛡️", layout="centered")
st.markdown("""
    <style>
    html, body, [class*="css"]  {
        background-color: #0f172a !important;
        color: white !important;
    }

    .stApp {
        background-color: rgba(255, 255, 255, 0.02) !important;
        padding: 2rem;
        border-radius: 16px;
        backdrop-filter: blur(12px);
    }

    .stCheckbox > div, .stTextInput > div > input, .stTextArea > div > textarea {
        background-color: rgba(255, 255, 255, 0.07) !important;
        color: white !important;
        border: none !important;
        border-radius: 8px;
    }

    .stCheckbox label {
        font-size: 1rem;
        color: white;
    }

    .stButton > button {
        background: linear-gradient(90deg, #22c55e, #2563eb);
        color: white;
        border: none;
        font-weight: bold;
        border-radius: 8px;
        padding: 0.75rem 1.5rem;
        transition: transform 0.2s ease;
    }

    .stButton > button:hover {
        transform: scale(1.03);
        background: linear-gradient(90deg, #4ade80, #3b82f6);
    }

    .stDownloadButton > button {
        background: linear-gradient(90deg, #3b82f6, #10b981);
        color: white;
        font-weight: bold;
        border: none;
        border-radius: 8px;
        padding: 0.6rem 1.2rem;
        margin-top: 1rem;
    }

    .stDownloadButton > button:hover {
        background: linear-gradient(90deg, #06b6d4, #10b981);
        transform: scale(1.03);
    }

    .stJson {
        background-color: rgba(255,255,255,0.05);
        padding: 1rem;
        border-radius: 8px;
        color: white;
    }

    </style>
""", unsafe_allow_html=True)

# 📋 Fields in order (MUST match Rust CLI VC message order)
FIELD_LABELS = {
    1: "👤 Full Name",
    2: "📱 Phone Number",
    3: "📧 Email",
    4: "🧾 Aadhar Number",
    5: "🎂 Date of Birth",
    6: "🏠 Address",
    7: "🆔 PAN Number"
}

st.title("🛡️ ConsentCast: Verifier Portal")
st.markdown("Select which fields you want the user to disclose for verification:")

# ✅ Tick boxes = indices
selected_indices = []
with st.form("field_selector"):
    for i, label in FIELD_LABELS.items():
        if st.checkbox(f"{label}", value=True):
            selected_indices.append(i)
    submit = st.form_submit_button("✅ Generate Proof")

# 🧠 On submit: POST to /generate-proof
if submit:
    st.info("⏳ Generating zero-knowledge proof...")

    try:
        res = requests.post("http://localhost:8060/generate-proof", json={
            "reveal_indices": selected_indices
        })

        if res.status_code == 200:
            proof_bundle = res.json()
            st.success("✅ Proof successfully generated!")
            st.json(proof_bundle)

            st.download_button(
                label="📥 Download Proof",
                data=json.dumps(proof_bundle, indent=2),
                file_name="proof_bundle.json",
                mime="application/json"
            )
        else:
            st.error("❌ Failed to generate proof")
            st.code(res.text)

    except Exception as e:
        st.error("🚨 Error connecting to backend")
        st.code(str(e))
