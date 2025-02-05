use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_681524: FileFormat = FileFormat {
    id: 681_524,
    source_type: SourceType::Wikidata,
    name: "XML Shareable Playlist Format",
    extensions: &["xspf"],
    media_types: &["application/xspf+xml"],
    signatures: &[],
    related_formats: &[],
};
