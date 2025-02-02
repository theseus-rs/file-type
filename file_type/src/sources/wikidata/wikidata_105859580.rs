use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_105859580: FileFormat = FileFormat {
    id: 105_859_580,
    source_type: SourceType::Wikidata,
    name: "ParaView VTK Unstructured grid",
    extensions: &["vtu"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
