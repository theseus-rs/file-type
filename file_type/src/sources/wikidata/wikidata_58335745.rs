use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_58335745: FileFormat = FileFormat {
    id: 58_335_745,
    source_type: SourceType::Wikidata,
    name: "Acrobat Catalog Cat File",
    extensions: &["cat"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
