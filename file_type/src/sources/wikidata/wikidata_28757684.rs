use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28757684: FileFormat = FileFormat {
    id: 28_757_684,
    source_type: SourceType::Wikidata,
    name: "Nintendo Game Boy cartridge SAV file",
    extensions: &["sav"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
