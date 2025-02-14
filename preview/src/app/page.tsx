import PriceUpdates from "@/components/PriceUpdates";

export default function Home() {
  return (
    <div className="min-h-screen p-8">
      <h1 className="text-2xl font-bold mb-4">Listen Data Service</h1>
      <PriceUpdates />
    </div>
  );
}
