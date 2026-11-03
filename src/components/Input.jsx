import React from "react";

export function Input({ label, placeholder, onSubmit, onSuccess }) {
  const handleSubmit = (e) => {
    e.preventDefault();
    const input = e.target.querySelector("input");
    const value = input.value.trim();
    if (value) {
      onSubmit(value);
      input.value = "";
    }
  };

  return (
    <form onSubmit={handleSubmit} className="space-y-3">
      <label className="block text-sm font-medium text-gray-700">
        {label}
      </label>
      <input
        type="text"
        placeholder={placeholder}
       className="w-full border border-gray-300 rounded-md px-3 py-2 focus:outline-none focus:ring-2 focus:ring-red-500 focus:border-transparent"
      />
      <Button type="submit">Submit</Button>
    </form>
  );
}
