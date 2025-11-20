# syntax=docker/dockerfile:1

FROM nixos/nix:latest AS rs_build

# Install packages using Nix
RUN nix-env -iA nixpkgs.gcc nixpkgs.bun nixpkgs.rustc nixpkgs.cargo

WORKDIR /app

# Copy source and asset directories
COPY . .

# build assets (adjust these commands as needed for your project)
RUN bun install

# Build JS
RUN bun scripts/build.ts

# build Rust app
RUN cargo build --release

# --- Runner stage ---
FROM nixos/nix:latest AS runner

WORKDIR /app

# Install runtime dependencies (if any)
RUN nix-env -iA nixpkgs.sqlite

# Copy built binary and assets from build stage
COPY --from=rs_build /app/target/release/sportsday-scoreboard-v2 /app/sportsday-scoreboard-v2
COPY --from=rs_build /app/assets /app/assets

COPY ./config.yaml /app/config.yaml

EXPOSE 3000
CMD ["/app/sportsday-scoreboard-v2"]