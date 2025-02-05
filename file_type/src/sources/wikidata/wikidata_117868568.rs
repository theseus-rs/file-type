use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117868568: FileFormat = FileFormat {
    id: 117_868_568,
    source_type: SourceType::Wikidata,
    name: "Imavox TurboFax file",
    extensions: &["tbf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
