use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_56827137: FileFormat = FileFormat {
    id: 56_827_137,
    source_type: SourceType::Wikidata,
    name: "Nintendo DS cartridge file",
    extensions: &["nds"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
