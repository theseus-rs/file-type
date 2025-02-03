use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_56827137: FileFormat = FileFormat {
    id: 56_827_137,
    source_type: SourceType::Wikidata,
    name: "Nintendo DS cartridge file",
    extensions: &["nds"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
