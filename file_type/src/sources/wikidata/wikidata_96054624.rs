use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_96054624: FileFormat = FileFormat {
    id: 96_054_624,
    source_type: SourceType::Wikidata,
    name: "Modelica model format",
    extensions: &["mo"],
    media_types: &["text/x-modelica"],
    signatures: &[],
    related_formats: &[],
};
