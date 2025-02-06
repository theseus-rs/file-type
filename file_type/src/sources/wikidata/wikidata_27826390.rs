use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27826390: FileFormat = FileFormat {
    id: 27_826_390,
    source_type: SourceType::Wikidata,
    name: "Proxy AUX file, AUX.XML variant",
    extensions: &["aux.xml"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
