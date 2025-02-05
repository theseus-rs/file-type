use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_55739507: FileFormat = FileFormat {
    id: 55_739_507,
    source_type: SourceType::Wikidata,
    name: "Genbox Family History file format",
    extensions: &["GDB"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
