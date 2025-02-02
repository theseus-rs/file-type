use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117401200: FileFormat = FileFormat {
    id: 117_401_200,
    source_type: SourceType::Wikidata,
    name: "Asymetrix Toolbook File 6-11.5",
    extensions: &["sbk", "tbk"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
