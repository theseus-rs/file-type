use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_12581295: FileFormat = FileFormat {
    id: 12_581_295,
    source_type: SourceType::Wikidata,
    name: "KT 3G video file format",
    extensions: &["k3g"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
