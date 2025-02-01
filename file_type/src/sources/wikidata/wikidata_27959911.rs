use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27959911: FileFormat = FileFormat {
    id: 27_959_911,
    puid: "wikidata/27959911",
    name: "ATRAC Advanced Lossless",
    extensions: &["aa3", "aal", "at3", "oma", "omg"],
    media_types: &[
        "audio/ATRAC-ADVANCED-LOSSLESS",
        "audio/ATRAC-ADVANCED-LOSSLESS",
        "audio/ATRAC-ADVANCED-LOSSLESS",
        "audio/ATRAC-ADVANCED-LOSSLESS",
        "audio/ATRAC-ADVANCED-LOSSLESS",
    ],
    internal_signatures: &[],
    related_formats: &[],
};
