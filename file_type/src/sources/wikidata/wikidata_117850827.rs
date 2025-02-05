use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117850827: FileFormat = FileFormat {
    id: 117_850_827,
    source_type: SourceType::Wikidata,
    name: "OAZ Fax file",
    extensions: &["oaz"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
