import streamlit as st
import json
import uuid
import datetime
import subprocess
import os

# Hide Streamlit chrome and style the page
st.set_page_config(page_title="Bank VC Issuer", page_icon="🏦", layout="centered")
st.markdown("""
  <style>
  #MainMenu, header, footer {visibility: hidden;}
  body::before {
    content: "";
    position: fixed;
    inset: 0;
    background: conic-gradient(
      from 0deg at 50% 50%,
      #ff3366, #ff9933, #66ff33, #33ccff, #9933ff, #ff3366
    );
    background-size: 400% 400%;
    animation: rotateBG 20s linear infinite;
    filter: blur(100px);
    z-index: -2;
  }
  @keyframes rotateBG {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
  }
  body::after {
    content: "";
    position: fixed;
    inset: 0;
    background: radial-gradient(circle at center, rgba(255,255,255,0.05), transparent 70%);
    z-index: -1;
  }
  .stApp {
    background: rgba(0,0,0,0.5);
    backdrop-filter: blur(15px);
    border-radius: 16px;
    padding: 2rem;
  }
  .stButton>button {
    background: linear-gradient(90deg, #00ffa3, #ff007c);
    color: white;
    font-weight: bold;
    border: none;
    border-radius: 8px;
    padding: 0.75rem 1.5rem;
    transition: transform 0.2s;
  }
  .stButton>button:hover {
    transform: scale(1.05);
  }
  </style>
""", unsafe_allow_html=True)

st.title(" Bank VC Issuer")
st.write("Generate a Verifiable Credential with BBS+ Signature")

# --- Form Section ---
with st.form("vc_form", clear_on_submit=True):
    name   = st.text_input("Full Name")
    ph_no  = st.text_input("Phone Number")
    email  = st.text_input("Email")
    aadhar = st.text_input("Aadhar Number")
    dob    = st.date_input("Date of Birth", min_value=datetime.date(1900,1,1))
    address = st.text_area("Address")
    pan     = st.text_input("PAN Number")

    submitted = st.form_submit_button("🚀 Issue VC")

# --- Submission logic outside the form ---
if submitted:
    vc_id = f"urn:uuid:{uuid.uuid4().hex[:24]}"
    issuance_date = datetime.datetime.utcnow().isoformat()

    vc_input = {
        "id": "1234",
        "name": name,
        "ph_no": ph_no,
        "email": email,
        "aadhar": aadhar,
        "dob": dob.isoformat(),
        "address": address,
        "pan": pan,
        "issuer": "did:web:bank.com",
        "issuance_date": issuance_date
    }

    with open("vc_input.json", "w") as f:
        json.dump(vc_input, f)

    result = subprocess.run(["./app/target/release/app"], capture_output=True, text=True)

    if result.returncode == 0 and os.path.exists("credential.json"):
        with open("credential.json") as f:
            vc = json.load(f)
        st.success("✅ Verifiable Credential issued successfully!")
        st.json(vc)

        st.download_button(
            label=" Download VC",
            data=json.dumps(vc, indent=2),
            file_name="credential.json",
            mime="application/json"
        )
    else:
        st.error("❌ Failed to issue VC.")
        st.code(result.stderr or "Unknown error")
