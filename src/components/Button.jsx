import React from "react";

export function Button({ onClick, loading, children, disabled }) {
  return (
    <button
      onClick={onClick}
      disabled={disabled || loading}
      className="bg-red-600 hover:bg-red-700 text-white font-medium px-4 py-2 rounded-md transition-colors disabled:opacity-50"
    >
      {loading ? "Processing..." : children}
    </button>
  );
}
