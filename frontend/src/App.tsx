import React, { useState } from 'react';
import axios from 'axios';
import './App.scss';

const getExplorerUrl = (blockchain: string, address: string) => {
  switch (blockchain.toLowerCase()) {
    case 'bitcoin':
      return `https://www.blockchain.com/btc/address/${address}`;
    case 'ethereum':
      return `https://etherscan.io/address/${address}`;
    case 'polygon':
      return `https://polygonscan.com/address/${address}`;
    case 'optimism':
      return `https://optimistic.etherscan.io/address/${address}`;
    case 'base':
      return `https://basescan.org/address/${address}`;
    case 'solana':
      return `https://explorer.solana.com/address/${address}`;
    case 'cardano':
      return `https://cardanoscan.io/address/${address}`;
    case 'sui':
      return `https://suiscan.xyz/address/${address}`;
    case 'aptos':
      return `https://explorer.aptoslabs.com/account/${address}`;
    default:
      return '';
  }
};

function App() {
  const [address, setAddress] = useState('');
  const [result, setResult] = useState<{status: string, blockchain?: string, error?: string} | null>(null);
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const validateInputs = () => {
    if (!address.trim()) {
      setError('Address field cannot be empty');
      return false;
    }
    setError(null);
    return true;
  };

  const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
    event.preventDefault();
    if (!validateInputs()) return;

    setIsLoading(true);
    setResult(null);
    try {
      const response = await axios.post('http://localhost:8080/validate', { address });
      setResult(response.data);
    } catch (error) {
      console.error('Error:', error);
      if (axios.isAxiosError(error)) {
        setResult({ 
          status: 'error', 
          error: error.response?.data?.message || 'An error occurred while validating the address'
        });
      } else {
        setResult({ status: 'error', error: 'An unknown error occurred' });
      }
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="container">
      <h1>Crypto Address Checker</h1>
      <form onSubmit={handleSubmit}>
        <label>
          Address:
          <input 
            type="text" 
            value={address} 
            onChange={(e) => setAddress(e.target.value)}
            placeholder="Enter crypto address"
          />
        </label>
        <button type="submit" disabled={isLoading}>
          {isLoading ? 'Validating...' : 'Validate'}
        </button>
      </form>

      {error && (
        <div className="error-message">
          {error}
        </div>
      )}

      {result && (
        <div className="result-container">
          <div className="card">
            <h2 className="card-header">Validation Result</h2>
            <div className="card-content">
              <p className={`status ${result.status}`}>
                Status: {result.status}
              </p>
              {result.blockchain && <p>Blockchain: {result.blockchain}</p>}
              {result.error && <p className="error">Error: {result.error}</p>}
            </div>
          </div>
          {result.blockchain && (
            <div className="card">
              <h2 className="card-header">Blockchain Explorer</h2>
              <div className="card-content">
                <a 
                  href={getExplorerUrl(result.blockchain, address)} 
                  target="_blank" 
                  rel="noopener noreferrer"
                  className="explorer-link"
                >
                  View on Explorer
                  <span className="external-link-icon">&#x2197;</span>
                </a>
              </div>
            </div>
          )}
        </div>
      )}
    </div>
  );
}

export default App;