use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27979253: FileFormat = FileFormat {
    id: 27_979_253,
    source_type: SourceType::Wikidata,
    name: "PCBoard",
    extensions: &["pcb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
