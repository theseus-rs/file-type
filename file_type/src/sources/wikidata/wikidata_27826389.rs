use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826389: FileFormat = FileFormat {
    id: 27_826_389,
    source_type: SourceType::Wikidata,
    name: "Proxy AUX file, AUX variant",
    extensions: &["aux"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
