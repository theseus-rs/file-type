use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_96054624: FileFormat = FileFormat {
    id: 96_054_624,
    source_type: SourceType::Wikidata,
    name: "Modelica model format",
    extensions: &["mo"],
    media_types: &["text/x-modelica"],
    internal_signatures: &[],
    related_formats: &[],
};
