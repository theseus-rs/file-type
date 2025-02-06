use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105047785: FileFormat = FileFormat {
    id: 105_047_785,
    source_type: SourceType::Wikidata,
    name: "Binary Color Format",
    extensions: &["bcf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
