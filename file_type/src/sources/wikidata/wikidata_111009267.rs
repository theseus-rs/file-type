use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111009267: FileFormat = FileFormat {
    id: 111_009_267,
    source_type: SourceType::Wikidata,
    name: "PrintMaster Newsletter File format",
    extensions: &["nws"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
