'use client';

import { useState, useEffect } from 'react';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, BarChart, Bar } from 'recharts';
import { TrendingUp, TrendingDown, Activity, DollarSign } from 'lucide-react';

interface TradeData {
  symbol: string;
  price: number;
  volume: number;
  side: 'Buy' | 'Sell';
  timestamp: string;
}

interface RsiData {
  symbol: string;
  rsi_value: number;
  signal: 'Overbought' | 'Oversold' | 'Neutral';
  timestamp: string;
}

export default function TradingDashboard() {
  const [prices, setPrices] = useState<Record<string, number>>({});
  const [rsiData, setRsiData] = useState<Record<string, number>>({});
  const [priceHistory, setPriceHistory] = useState<Array<{symbol: string, price: number, timestamp: string}>>([]);
  const [isConnected, setIsConnected] = useState(false);
  const [isClient, setIsClient] = useState(false);

  useEffect(() => {
    setIsClient(true);
  }, []);

  useEffect(() => {
    const fetchData = async () => {
      try {
        const [pricesRes, rsiRes] = await Promise.all([
          fetch('http://localhost:3001/prices').catch(() => ({ json: () => ({}) })),
          fetch('http://localhost:3001/rsi').catch(() => ({ json: () => ({}) }))
        ]);
        
        const pricesData = await pricesRes.json();
        const rsiData = await rsiRes.json();
        
        if (Object.keys(pricesData).length > 0) {
          setPrices(pricesData);
          setRsiData(rsiData);
          setIsConnected(true);
          
          // Add to price history for chart
          Object.entries(pricesData).forEach(([symbol, price]) => {
            setPriceHistory(prev => [...prev.slice(-50), {
              symbol,
              price: price as number,
              timestamp: new Date().toLocaleTimeString()
            }]);
          });
        } else {
          setIsConnected(false);
        }
      } catch (error) {
        console.error('Error fetching data:', error);
        setIsConnected(false);
      }
    };

    fetchData();
    const interval = setInterval(fetchData, 2000); // Update every 2 seconds
    
    return () => clearInterval(interval);
  }, []);

  const getRsiColor = (rsi: number) => {
    if (rsi >= 70) return 'text-red-500';
    if (rsi <= 30) return 'text-green-500';
    return 'text-yellow-500';
  };

  const getRsiSignal = (rsi: number) => {
    if (rsi >= 70) return 'Overbought';
    if (rsi <= 30) return 'Oversold';
    return 'Neutral';
  };

  if (!isClient) {
    return (
      <div className="min-h-screen bg-gray-900 text-white p-6 flex items-center justify-center">
        <div className="text-center">
          <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
          <p className="text-xl">Loading Trading Dashboard...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gray-900 text-white p-6">
      <div className="max-w-7xl mx-auto">
        <div className="flex items-center justify-between mb-8">
          <h1 className="text-4xl font-bold bg-gradient-to-r from-blue-400 to-purple-500 bg-clip-text text-transparent">
            Real-Time Trading Analytics
          </h1>
          <div className="flex items-center space-x-2">
            <div className={`w-3 h-3 rounded-full ${isConnected ? 'bg-green-500' : 'bg-red-500'}`}></div>
            <span className="text-sm">{isConnected ? 'Connected' : 'Disconnected'}</span>
          </div>
        </div>

        {/* Stats Cards */}
        <div className="grid grid-cols-1 md:grid-cols-4 gap-6 mb-8">
          <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Active Symbols</p>
                <p className="text-2xl font-bold">{Object.keys(prices).length}</p>
              </div>
              <Activity className="w-8 h-8 text-blue-400" />
            </div>
          </div>
          
          <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Total Volume</p>
                <p className="text-2xl font-bold">
                  {Object.values(prices).reduce((sum, price) => sum + price, 0).toFixed(0)}
                </p>
              </div>
              <DollarSign className="w-8 h-8 text-green-400" />
            </div>
          </div>
          
          <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Overbought</p>
                <p className="text-2xl font-bold text-red-400">
                  {Object.values(rsiData).filter(rsi => rsi >= 70).length}
                </p>
              </div>
              <TrendingUp className="w-8 h-8 text-red-400" />
            </div>
          </div>
          
          <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <div className="flex items-center justify-between">
              <div>
                <p className="text-gray-400 text-sm">Oversold</p>
                <p className="text-2xl font-bold text-green-400">
                  {Object.values(rsiData).filter(rsi => rsi <= 30).length}
                </p>
              </div>
              <TrendingDown className="w-8 h-8 text-green-400" />
            </div>
          </div>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          {/* Price Chart */}
          <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <h2 className="text-xl font-semibold mb-4">Price Movement</h2>
            <ResponsiveContainer width="100%" height={300}>
              <LineChart data={priceHistory}>
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                <XAxis dataKey="timestamp" stroke="#9CA3AF" />
                <YAxis stroke="#9CA3AF" />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1F2937', 
                    border: '1px solid #374151',
                    borderRadius: '8px'
                  }} 
                />
                <Line 
                  type="monotone" 
                  dataKey="price" 
                  stroke="#3B82F6" 
                  strokeWidth={2}
                  dot={{ fill: '#3B82F6', strokeWidth: 2, r: 4 }}
                />
              </LineChart>
            </ResponsiveContainer>
          </div>

          {/* RSI Chart */}
          <div className="bg-gray-800 rounded-lg p-6 border border-gray-700">
            <h2 className="text-xl font-semibold mb-4">RSI Indicators</h2>
            <ResponsiveContainer width="100%" height={300}>
              <BarChart data={Object.entries(rsiData).map(([symbol, rsi]) => ({ symbol, rsi }))}>
                <CartesianGrid strokeDasharray="3 3" stroke="#374151" />
                <XAxis dataKey="symbol" stroke="#9CA3AF" />
                <YAxis stroke="#9CA3AF" domain={[0, 100]} />
                <Tooltip 
                  contentStyle={{ 
                    backgroundColor: '#1F2937', 
                    border: '1px solid #374151',
                    borderRadius: '8px'
                  }} 
                />
                <Bar 
                  dataKey="rsi" 
                  fill="#8B5CF6"
                  radius={[4, 4, 0, 0]}
                />
              </BarChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Live Data Table */}
        <div className="mt-8 bg-gray-800 rounded-lg border border-gray-700">
          <div className="p-6 border-b border-gray-700">
            <h2 className="text-xl font-semibold">Live Market Data</h2>
          </div>
          <div className="overflow-x-auto">
            <table className="w-full">
              <thead className="bg-gray-700">
                <tr>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Symbol</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Price</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">RSI</th>
                  <th className="px-6 py-3 text-left text-xs font-medium text-gray-300 uppercase tracking-wider">Signal</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-gray-700">
                {Object.entries(prices).map(([symbol, price]) => (
                  <tr key={symbol} className="hover:bg-gray-700">
                    <td className="px-6 py-4 whitespace-nowrap text-sm font-medium">{symbol}</td>
                    <td className="px-6 py-4 whitespace-nowrap text-sm">${price.toFixed(2)}</td>
                    <td className={`px-6 py-4 whitespace-nowrap text-sm font-medium ${getRsiColor(rsiData[symbol] || 0)}`}>
                      {(rsiData[symbol] || 0).toFixed(2)}
                    </td>
                    <td className="px-6 py-4 whitespace-nowrap">
                      <span className={`inline-flex px-2 py-1 text-xs font-semibold rounded-full ${
                        getRsiSignal(rsiData[symbol] || 0) === 'Overbought' ? 'bg-red-900 text-red-300' :
                        getRsiSignal(rsiData[symbol] || 0) === 'Oversold' ? 'bg-green-900 text-green-300' :
                        'bg-yellow-900 text-yellow-300'
                      }`}>
                        {getRsiSignal(rsiData[symbol] || 0)}
                      </span>
                    </td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}