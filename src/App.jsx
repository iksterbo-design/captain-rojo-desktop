import React from "react";
import { invoke } from "@tauri-apps/api/core";
import { useState } from "react";
import { Button } from "./components/Button";
import { Input } from "./components/Input";
import { Card } from "./components/Card";

function App() {
  const [status, setStatus] = useState("idle");
  const [result, setResult] = useState("");
  const [loading, setLoading] = useState(false);
  const [health, setHealth] = useState(null);

  const runHealth = async () => {
    setLoading(true);
    try {
      const res = await invoke<string>("health_check");
      setHealth(JSON.parse(res));
      setResult(res);
    } catch (e) {
      setResult("Error: " + e);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col">
      <header className="bg-gradient-to-r from-red-600 to-red-500 text-white px-6 py-4">
        <h1 className="text-2xl font-bold">Captain Rojo</h1>
        <p className="text-sm opacity-90">AI Branding & Ad-Production Studio</p>
      </header>

      <main className="flex-1 container mx-auto px-6 py-8 max-w-4xl">
        {/* Health card */}
        <Card title="Environment Health">
          <Button onClick={runHealth} loading={loading}>
            {loading ? "Checking..." : "Run Health Check"}
          </Button>
          {health && (
            <div className="mt-4 space-y-2">
              <p className="text-sm">
                <span className="font-semibold">Python:</span> {health.python}
              </p>
              <p className="text-sm">
                <span className="font-semibold">ffmpeg:</span> {health.ffmpeg}
              </p>
              <p className="text-sm">
                <span className="font-semibold">piper:</span> {health.piper}
              </p>
              <p className="text-sm">
                <span className="font-semibold">Cloudflare:</span> {health.cloudflare}
              </p>
              <p className="text-sm">
                <span className="font-semibold">Model:</span> {health.model}
              </p>
            </div>
          )}
        </Card>

        {/* Logo Studio */}
        <Card title="Logo Studio">
          <Input
            label="Logo prompt"
            placeholder="A bold red dragon with golden wings, minimal line art style"
            onSubmit={(prompt) => {
              setLoading(true);
              invoke("logo_gen", { prompt, style: "minimal", mode: "local" })
                .then((r) => setResult(r))
                .catch((e) => setResult("Error: " + e))
                .finally(() => setLoading(false));
            }}
            onSuccess={(r) => setResult(r)}
          />
        </Card>

        {/* Ad Studio */}
        <Card title="Ad Studio">
          <Input
            label="Ad prompt"
            placeholder="Cinematic product launch video, 3 scenes, dramatic lighting"
            onSubmit={(prompt) => {
              setLoading(true);
              invoke("ad_gen", { prompt, scenes: "3" })
                .then((r) => setResult(r))
                .catch((e) => setResult("Error: " + e))
                .finally(() => setLoading(false));
            }}
            onSuccess={(r) => setResult(r)}
          />
        </Card>

        {/* Results */}
        {result && (
          <Card title="Result">
            <pre className="text-sm bg-gray-900 text-green-200 p-4 rounded overflow-auto max-h-60">
              {result}
            </pre>
          </Card>
        )}
      </main>

      <footer className="bg-gray-100 text-center text-xs text-gray-500 py-4">
        Captain Rojo Desktop v1.0 — Local AI toolkit integration
      </footer>
    </div>
  );
}

export default App;
