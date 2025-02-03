use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_117868568: FileFormat = FileFormat {
    id: 117_868_568,
    source_type: SourceType::Wikidata,
    name: "Imavox TurboFax file",
    extensions: &["tbf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
