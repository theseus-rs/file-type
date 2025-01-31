use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_96054624: FileFormat = FileFormat {
    id: 96_054_624,
    puid: "wikidata/96054624",
    name: "Modelica model format",
    extensions: &["mo"],
    media_types: &["text/x-modelica"],
    internal_signatures: &[],
    related_formats: &[],
};
