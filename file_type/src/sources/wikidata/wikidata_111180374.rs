use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111180374: FileFormat = FileFormat {
    id: 111_180_374,
    source_type: SourceType::Wikidata,
    name: "PressWriter File",
    extensions: &["psp"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
