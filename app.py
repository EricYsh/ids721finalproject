from transformers import pipeline
import sys
import json

def summarize_text(text):
    try:
        summarizer = pipeline(task="summarization", model="Falconsai/text_summarization")
        summarization = summarizer(text, max_length=25, min_length=10, do_sample=False)
        return summarization
    except Exception as e:
        return {"error": str(e)}

if __name__ == '__main__':
    if len(sys.argv) < 1:
        print("Usage: python app.py <input_text>")
        sys.exit(1)

    input_text = sys.argv[1]

    summarized_results = summarize_text(input_text)
    print(json.dumps(summarized_results))
