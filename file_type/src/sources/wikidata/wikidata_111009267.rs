use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111009267: FileFormat = FileFormat {
    id: 111_009_267,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Newsletter File format",
    extensions: &["nws"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
