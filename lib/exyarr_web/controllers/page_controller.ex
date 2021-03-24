defmodule ExyarrWeb.PageController do
  use ExyarrWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html", login_url: Exyarr.Oauth.authorize_uri())
  end

  def oauth_redirect(conn, %{"code" => code}) do
    %{"access_token" => access_token, "refresh_token" => refresh_token} =
      Exyarr.Oauth.request_tokens(code)

    user_id =
      Exyarr.Spotify.get!("/me", Authorization: "Bearer #{access_token}").body[
        "id"
      ]

    user = %Exyarr.User{
      access_token: access_token,
      refresh_token: refresh_token,
      spotify_id: user_id
    }

    Exyarr.Repo.insert!(user)

    conn
    |> put_flash(:info, "Connected!")
    |> render("connected.html")
  end
end
