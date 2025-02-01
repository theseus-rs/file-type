use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_12581295: FileFormat = FileFormat {
    id: 12_581_295,
    puid: "wikidata/12581295",
    name: "KT 3G video file format",
    extensions: &["k3g"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
