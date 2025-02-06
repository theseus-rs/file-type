use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_125045112: FileFormat = FileFormat {
    id: 125_045_112,
    source_type: SourceType::Wikidata,
    name: "Yoshimi Patch Set File",
    extensions: &["xmz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
