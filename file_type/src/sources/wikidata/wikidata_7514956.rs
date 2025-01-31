use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_7514956: FileFormat = FileFormat {
    id: 7_514_956,
    puid: "wikidata/7514956",
    name: "Silicon Graphics Image",
    extensions: &[
        "bw", "bw", "bw", "bw", "rgb", "rgb", "rgb", "rgb", "rgba", "rgba", "rgba", "rgba", "sgi",
        "sgi", "sgi", "sgi",
    ],
    media_types: &[
        "image/sgi",
        "image/sgi",
        "image/sgi",
        "image/sgi",
        "image/x-rgb",
        "image/x-rgb",
        "image/x-rgb",
        "image/x-rgb",
        "image/x-sgi",
        "image/x-sgi",
        "image/x-sgi",
        "image/x-sgi",
        "image/x-sgi-rgb",
        "image/x-sgi-rgb",
        "image/x-sgi-rgb",
        "image/x-sgi-rgb",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
