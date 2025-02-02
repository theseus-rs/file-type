use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117456477: FileFormat = FileFormat {
    id: 117_456_477,
    source_type: SourceType::Wikidata,
    name: "Microsoft Access Database File 1.1",
    extensions: &["mda", "mdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
