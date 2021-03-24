defmodule Exyarr.Oauth do
  def autorize_uri() do
    spotify_base_url = "https://accounts.spotify.com/authorize"

    client_id =
      Application.fetch_env!(:exyarr, :spotify_client_id)
      |> URI.encode()

    redirect_uri =
      ExyarrWeb.Router.Helpers.page_url(ExyarrWeb.Endpoint, :oauth_redirect)
      |> URI.encode()

    scopes =
      "user-follow-read playlist-modify-private playlist-read-private playlist-modify-public"
      |> URI.encode()

    """
    #{spotify_base_url}
    ?response_type=code
    &client_id=#{client_id}
    &redirect_uri=#{redirect_uri}
    &scope=#{scopes}
    """
  end
end
