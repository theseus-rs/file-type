use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117401200: FileFormat = FileFormat {
    id: 117_401_200,
    source_type: SourceType::Wikidata,
    name: "Asymetrix Toolbook File 6-11.5",
    extensions: &["sbk", "tbk"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
