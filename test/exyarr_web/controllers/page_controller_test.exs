defmodule ExyarrWeb.PageControllerTest do
  use ExyarrWeb.ConnCase

  test "GET /", %{conn: conn} do
    conn = get(conn, "/")

    escaped_uri =
      Exyarr.Oauth.authorize_uri()
      |> Phoenix.HTML.html_escape()
      |> Phoenix.HTML.safe_to_string()

    assert html_response(conn, 200) =~ escaped_uri
  end
end
