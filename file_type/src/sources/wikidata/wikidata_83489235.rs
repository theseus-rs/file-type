use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_83489235: FileFormat = FileFormat {
    id: 83_489_235,
    source_type: SourceType::Wikidata,
    name: "VisiCalc file format",
    extensions: &["vc", "vcs"],
    media_types: &["application/octet-stream"],
    signatures: &[],
    related_formats: &[],
};
