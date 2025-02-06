use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129425911: FileFormat = FileFormat {
    id: 129_425_911,
    source_type: SourceType::Wikidata,
    name: "Gosu template file",
    extensions: &["gst"],
    media_types: &["text/x-gosu-template"],
    signatures: &[],
    related_formats: &[],
};
