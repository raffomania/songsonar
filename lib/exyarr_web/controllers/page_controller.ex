defmodule ExyarrWeb.PageController do
  use ExyarrWeb, :controller

  def index(conn, _params) do
    render(conn, "index.html", login_url: Exyarr.Oauth.autorize_uri())
  end

  def oauth_redirect(conn, %{"code" => code}) do
    IO.inspect(code)

    conn
    |> put_flash(:info, "Connected!")
    |> render("connected.html")
  end
end
