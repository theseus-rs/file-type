use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47895228: FileFormat = FileFormat {
    id: 47_895_228,
    source_type: SourceType::Wikidata,
    name: "Visual Basic Macro",
    extensions: &["dvb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
