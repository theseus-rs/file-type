use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_55739507: FileFormat = FileFormat {
    id: 55_739_507,
    source_type: SourceType::Wikidata,
    name: "Genbox Family History file format",
    extensions: &["GDB"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
