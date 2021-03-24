defmodule Exyarr.Oauth do
  def redirect_uri() do
    ExyarrWeb.Router.Helpers.page_url(ExyarrWeb.Endpoint, :oauth_redirect)
    |> URI.encode()
  end

  def authorize_uri() do
    spotify_base_url = "https://accounts.spotify.com/authorize"

    client_id =
      Application.fetch_env!(:exyarr, :spotify_client_id)
      |> URI.encode()

    scopes =
      "user-follow-read playlist-modify-private playlist-read-private playlist-modify-public"
      |> URI.encode()

    """
    #{spotify_base_url}
    ?response_type=code
    &client_id=#{client_id}
    &redirect_uri=#{redirect_uri()}
    &scope=#{scopes}
    """
    |> String.replace("\n", "")
  end

  def request_tokens(code) do
    client_id = Application.fetch_env!(:exyarr, :spotify_client_id)
    client_secret = Application.fetch_env!(:exyarr, :spotify_client_secret)

    body =
      {:form,
       [
         code: code,
         grant_type: "authorization_code",
         redirect_uri: redirect_uri()
       ]}

    HTTPoison.post!("https://accounts.spotify.com/api/token", body, [],
      hackney: [basic_auth: {client_id, client_secret}]
    ).body
    |> Poison.decode!()
  end
end
