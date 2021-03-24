defmodule Exyarr.OauthTest do
  use ExUnit.Case

  test "no newlines in oauth URI" do
    refute Exyarr.Oauth.autorize_uri() =~ "\n"
  end
end
