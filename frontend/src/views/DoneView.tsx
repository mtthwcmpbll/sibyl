import { AssetImage } from "../components/AssetImage";
import type { WizardState } from "../state/wizard";

/** Final state: the approved deck identity, ready for future features. */
export function DoneView({ state }: { state: WizardState }) {
  const deck = state.deck;
  if (!deck) return null;
  return (
    <section className="view done-view" data-testid="done-view">
      <h2>This is your deck ✨</h2>
      <p className="muted">Deck {deck.id} is now active and ready for future readings.</p>
      <div className="deck-suits">
        {deck.suitIcons.map((icon) => (
          <figure key={icon.suit}>
            <AssetImage assetKey={icon.imageKey} alt={`${icon.suit} icon`} className="suit-icon" />
            <figcaption>{icon.suit}</figcaption>
          </figure>
        ))}
      </div>
    </section>
  );
}
