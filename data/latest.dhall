-- Latest planetary ephemerides, planetary constants, high precision Moon rotation, and daily Earth orientation parameter
{ files =
  [ { crc32 = Some 1921414410
    , uri = "http://public-data.nyxspace.com/anise/de440s.bsp"
    }
  , { crc32 = Some 1216081528
    , uri = "http://public-data.nyxspace.com/anise/pck08.pca"
    }
  , { crc32 = Some 1817759242
    , uri = "http://public-data.nyxspace.com/anise/moon_pa_de440_200625.bpc"
    }
  , { crc32 = None Natural
    , uri =
        "https://naif.jpl.nasa.gov/pub/naif/generic_kernels/pck/earth_latest_high_prec.bpc"
    }
  ]
}
