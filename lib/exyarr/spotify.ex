defmodule Exyarr.Spotify do
  use HTTPoison.Base

  @endpoint "https://api.spotify.com/v1"

  def process_url(url) do
    @endpoint <> url
  end

  def process_request_headers(headers) do
    case Keyword.fetch(headers, :access_token) do
      {:ok, token} -> Keyword.put(headers, :Authorization, "Bearer #{token}")
    end
  end

  def process_response_body(body) do
    body
    |> Poison.decode!()
  end
end
