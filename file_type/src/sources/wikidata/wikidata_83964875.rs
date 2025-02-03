use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_83964875: FileFormat = FileFormat {
    id: 83_964_875,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Workgroup Information File",
    extensions: &["mdw"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
