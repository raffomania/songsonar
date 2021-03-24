defmodule ExyarrWeb.PageController do
  use ExyarrWeb, :controller

  def index(conn, _params) do
    
    render(conn, "index.html")
  end
end
