from main import trilhas

def test_trilhas_carregadas():
    assert len(trilhas) > 0
    assert "1" in trilhas