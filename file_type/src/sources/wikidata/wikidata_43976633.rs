use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_43976633: FileFormat = FileFormat {
    id: 43_976_633,
    source_type: SourceType::Wikidata,
    name: "Exchangeable Image File Format (Audio)",
    extensions: &["wav"],
    media_types: &["audio/x-wav"],
    signatures: &[],
    related_formats: &[],
};
