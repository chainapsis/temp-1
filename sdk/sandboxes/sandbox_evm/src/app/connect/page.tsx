"use client";

import { useState } from "react";

// TODO: remove this page
export default function ConnectPopup() {
  const [result, setResult] = useState("");

  const handleConfirm = () => {
    if (window.opener) {
      window.opener.postMessage({ type: "test_result", result }, window.origin);
      window.close();
    }
  };

  return (
    <div
      style={{
        minHeight: "100vh",
        background: "linear-gradient(135deg, #e3ecff 0%, #f8fafc 100%)",
        display: "flex",
        alignItems: "center",
        justifyContent: "center",
      }}
    >
      <div
        style={{
          background: "#fff",
          borderRadius: 16,
          boxShadow: "0 4px 24px rgba(0,0,0,0.08)",
          padding: 32,
          minWidth: 340,
          display: "flex",
          flexDirection: "column",
          alignItems: "center",
        }}
      >
        <div style={{ fontSize: 40, marginBottom: 8 }}>🦾</div>
        <h2
          style={{
            margin: 0,
            marginBottom: 8,
            fontWeight: 700,
            fontSize: 22,
            color: "#0c2f78",
            letterSpacing: 0.5,
          }}
        >
          Keplr E-Wallet Connect
        </h2>
        <p
          style={{
            color: "#555",
            marginBottom: 24,
            fontSize: 15,
            textAlign: "center",
          }}
        >
          Enter a result message to send back to the parent window for testing
          the popup flow.
        </p>
        <input
          type="text"
          placeholder="Enter result message"
          value={result}
          onChange={(e) => setResult(e.target.value)}
          style={{
            marginBottom: 20,
            padding: "10px 14px",
            width: "100%",
            border: "1px solid #d1d5db",
            color: "#000",
            borderRadius: 8,
            fontSize: 16,
            outline: "none",
            transition: "border 0.2s",
            boxSizing: "border-box",
          }}
        />
        <button
          onClick={handleConfirm}
          style={{
            padding: "10px 0",
            width: "100%",
            background: "#0c2f78",
            color: "#fff",
            border: "none",
            borderRadius: 8,
            fontWeight: 600,
            fontSize: 16,
            cursor: "pointer",
            boxShadow: "0 2px 8px rgba(12,47,120,0.08)",
            transition: "background 0.2s",
          }}
        >
          Confirm
        </button>
      </div>
    </div>
  );
}
