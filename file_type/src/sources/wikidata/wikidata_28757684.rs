use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_28757684: FileFormat = FileFormat {
    id: 28_757_684,
    source_type: SourceType::Wikidata,
    name: "Nintendo Game Boy cartridge SAV file",
    extensions: &["sav"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
