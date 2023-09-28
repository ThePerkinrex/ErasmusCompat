import React, { useEffect, useState } from 'react';
import logo from './logo.svg';
import './App.scss';
import 'leaflet/dist/leaflet.css'
import leaflet_icon from 'leaflet/dist/images/marker-icon.png'
import leaflet_icon_shadow from 'leaflet/dist/images/marker-shadow.png'
import leaflet_icon_retina from 'leaflet/dist/images/marker-icon-2x.png'
import { MapContainer, Marker, Popup, TileLayer } from 'react-leaflet';
import { LatLngExpression, Icon } from 'leaflet';
import AddDestinations from './AddDestinations';
Icon.Default.mergeOptions({
  iconRetinaUrl: leaflet_icon_retina,
  iconUrl: leaflet_icon,
  shadowUrl: leaflet_icon_shadow,
})
const position: LatLngExpression = [51.505, -0.09]

interface Uni {
  city: string,
  country: string,
  lat: number | null
  lon: number | null,
  name: string,
  number: number,
  region: string,
}

type State = {
  unis: Uni[]
}

function App() {
  let [state, setState] = useState<State>({unis: []})
  // useEffect(() => {
  //   fetch("/api/unis").then(x => x.json()).then(unis => setState(s => ({...s, unis}))).catch(e => console.error("Error fetching unis: " + e))
  // }, [])
  return (
    <div className="App">
      {/* <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <a
          className="App-link"
          href="https://reactjs.org"
          target="_blank"
          rel="noopener noreferrer"
        >
          Learn React
        </a>
      </header> */}
      <MapContainer center={position} zoom={13} scrollWheelZoom={false} className='map'>
        <TileLayer
          attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
          url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
        />
        {state.unis.filter(uni => uni.lat !== null && uni.lon !== null).map((uni, idx) => <Marker position={[uni.lat as number, uni.lon as number]} key={`${uni.country} ${uni.region} ${uni.number}`}>
          <Popup>
            {uni.country} {uni.region} {uni.number}: {uni.name} @ {uni.city}
          </Popup>
        </Marker>)}
        {/* <Marker position={position}>
          <Popup>
            A pretty CSS3 popup. <br /> Easily customizable.
          </Popup>
        </Marker> */}
      </MapContainer>
      <AddDestinations/>
    </div>
  );
}

export default App;
