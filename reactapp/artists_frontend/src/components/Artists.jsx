import { useState, useEffect } from "react";
import axios from "axios";

function Artists() {
  const [artists, setArtists] = useState([]); // State für die Künstlerdaten
  const [loading, setLoading] = useState(true); // State für den Ladezustand
  const [error, setError] = useState(null); // State für Fehler

  useEffect(() => {
    // API-Aufruf
    axios
      .get("http://127.0.0.1:3000/api/artists")
      .then((response) => {
        setArtists(response.data); // Daten in den State speichern
        setLoading(false); // Ladezustand beenden
      })
      .catch(() => {
        setError("Fehler beim Laden der Daten");
        setLoading(false); // Ladezustand beenden
      });
  }, []); // Leeres Array -> nur beim Mount ausführen

  if (loading) return <p>Lädt...</p>; // Ladeanzeige
  if (error) return <p>{error}</p>; // Fehleranzeige

  return (
    <div>
      <h1>Künstler</h1>
      <ul>
        {artists.map((artist) => (
          <li key={artist.id}>
            <h2>{artist.name}</h2>
            <p>Email: {artist.email}</p>
          </li>
        ))}
      </ul>
    </div>
  );
}

export default Artists;
